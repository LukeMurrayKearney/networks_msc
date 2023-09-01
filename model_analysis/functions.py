import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.stats import poisson, geom, gmean
import math
import re

import seaborn as sns
plt.rcParams.update({'font.size':20})

def dataframe_to_contacts_vec(df_tmp):
    """
    Function turning data frame of contacts into numpy array of contacts numbers
    Input: Dataframe
    Output: Numpy Array 
    """
    contacts_tmp = df_tmp["part_id"].value_counts()
    for i, x in contacts_tmp.items():
        if x == 1: 
            idx = int(np.where(df_tmp["part_id"] == i)[0][0])
            if idx in df_tmp.index and pd.isna(df_tmp["cont_id"][idx]):
                contacts_tmp[i] = 0
    return contacts_tmp.values

def plot_degree_dist(contacts, params = None, idx = [0,0]):
    """
    Plot degree distribution and subsequent fit
    Input: Contacts Numpy array, parameters of fitting, index in subplot
    Output: Figure
    """
    if len(contacts) == 0:
        print(f'No contacts in {idx}')
        return
    unique = np.unique(np.array(contacts)+1,return_counts=True)
    plt.figure(figsize=(10,7))
    ax = plt.gca()
    ax.scatter(unique[0], unique[1]/sum(unique[1]), label="Data")
    if params != None or params != (0,0,0):
        max_x = np.max(contacts)
        x = np.arange(0, max_x + 1)
        pmf_pois = poisson.pmf(x, params[0])*params[2]
        pmf_geom = geom.pmf(x,params[1])*(1-params[2])
        ax.plot(x+1, pmf_geom+pmf_pois, 'ro-', lw=0.5,label="Fitted Poisson-Geometric Mixture")
    ax.set_ylim([min(unique[1]/sum(unique[1])),1])
    ax.set_yscale('log')
    ax.set_xscale('log')
    ax.set_ylabel("Number of participants")
    ax.set_xlabel("Number of contacts")
    ax.legend()
    # plt.savefig("../../figures/pre_processing_CoMix/fitting/multinom_fits2/degree_dist" + str(idx[0]) + str(idx[1]) + ".png",
                # bbox_inches="tight")
    plt.show()
    # plt.close()

def log_bins(contacts, A):
    """
    Returns log bins of contacts, A^m
    Input: Contacts -> np array, A -> float, m -> float
    Output: Geometric center of bins -> ndarray, values in bins -> ndarray
    """
    ms = [0]
    values = [0]
    contacts = np.sort(contacts)
    # count number of individuals in each bin
    i = 0
    while i < len(contacts):
        # if in current bin
        if contacts[i] < math.pow(A,ms[-1]):
            values[-1] += 1
        else:
            values.append(1)
            ms.append(ms[-1] + 1)
        i += 1
        # print(i, ms)
    ms.append(ms[-1] + 1)
    bin_centers = np.array([gmean(list(range(int(math.pow(A,ms[j])), int(math.pow(A,x))))) for j, x in enumerate(ms[1:])])
    bin_centers
    bin_centers[0] = 0 
    values = np.array(values) / np.sum(values)
    return bin_centers, values

def calc_age_mat(df_filter, buckets):
    n = len(buckets)+1
    result = np.zeros((n,n))
    num_participants = np.zeros(n)
    last = ""
    for part_idx, x in enumerate(df_filter.part_average_age):
        if np.isnan(x) or np.isnan(df_filter.cnt_average_age[part_idx]):
            continue
        i = -1; j = -1
        for k, top in enumerate(buckets):
            if top > df_filter.cnt_average_age[part_idx]:
                j = k
                break
        for k, top in enumerate(buckets):
            if top > x:
                i = k
                break
        result[i,j] += 1
    for l, x in enumerate(df_filter.part_id):
        if x == last:
            continue
        i = -1
        for k, top in enumerate(buckets):
            if top > df_filter.part_average_age[l]:
                i = k
                break
        num_participants[i] += 1
        last = x
    for i in range(len(buckets)+1):
        result[i] /= num_participants[i]
    return result 
    

def plot_age_mat(age_mat, buckets):
    age_mat = np.flip(age_mat, axis=0)
    bucket_labels = ["0-"+str(buckets[0]-1)] + [str(buckets[i-1]) + "-" + str(buckets[i]-1) for i in range(1,len(buckets))] + [str(buckets[-1]) + "+"]
    fig, ax = plt.subplots(1,1)
    fig.set_size_inches((15,11))
    sns.heatmap(age_mat, annot=True,square=True, cbar=True,cmap='RdBu')
    ax.set_ylabel("Contact Age Group")
    ax.set_xlabel("Participant Age Group")
    ax.set_xticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=bucket_labels,fontsize=18)
    ax.set_yticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=np.flip(bucket_labels),fontsize=18)
    # ax.collections[0].set_clim(0,np.max(age_mat))
    ax.collections[0].set_clim(-np.max(age_mat),np.max(age_mat))
    fig.tight_layout()
    fig.savefig("../../../../figures/important/4.fixing/stubs_missing1.eps", format='eps')
    plt.show()

def plot_age_mat2(age_mat, age_mat2, buckets):
    age_mat = np.flip(age_mat, axis=0)
    bucket_labels = ["0-"+str(buckets[0]-1)] + [str(buckets[i-1]) + "-" + str(buckets[i]-1) for i in range(1,len(buckets))] + [str(buckets[-1]) + "+"]
    fig, ax = plt.subplots(1,2)
    fig.set_size_inches((30,12))
    sns.heatmap(age_mat, annot=True,cmap='RdBu',square=True, cbar=True,ax=ax[0],fmt=".2f")
    ax[0].set_ylabel("Contact Age Group")
    ax[0].set_xlabel("Participant Age Group")
    ax[0].set_xticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=bucket_labels,fontsize=18)
    ax[0].set_yticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=np.flip(bucket_labels),fontsize=18)
    # ax.collections[0].set_clim(0,np.max(age_mat))
    ax[0].collections[0].set_clim(-np.max(age_mat),np.max(age_mat))
    ax[0].set_title("Period 1")

    age_mat2 = np.flip(age_mat2, axis=0)
    sns.heatmap(age_mat2, annot=True,square=True, cbar=True,cmap='RdBu',ax=ax[1],fmt=".2f")
    ax[1].set_ylabel("Contact Age Group")
    ax[1].set_xlabel("Participant Age Group")
    ax[1].set_xticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=bucket_labels,fontsize=18)
    ax[1].set_yticks(ticks= np.array(list(range(len(buckets)+1))) + 0.5,labels=np.flip(bucket_labels),fontsize=18)
    # ax.collections[0].set_clim(0,np.max(age_mat))
    ax[1].collections[0].set_clim(-np.max(age_mat),np.max(age_mat))
    ax[1].set_title("Period 2")
    fig.tight_layout()
    # fig.savefig("../../../../figures/important/4.fixing/fixed_stubs_missing.eps", format='eps',bbox_inches="tight")
    plt.show()
